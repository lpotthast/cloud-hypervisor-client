use assertr::prelude::*;
use rootcause::prelude::*;
use std::sync::LazyLock;
use yaml_rust2::yaml::Hash;
use yaml_rust2::{Yaml, YamlEmitter, YamlLoader};

static TYPE: LazyLock<Yaml> = LazyLock::new(|| Yaml::String("type".to_string()));
static FORMAT: LazyLock<Yaml> = LazyLock::new(|| Yaml::String("format".to_string()));

/// Integer formats lifted into the schema's `type:` key by the normalization pass.
const INT_FORMATS: &[&str] = &[
    "int8", "int16", "int32", "int64", "uint8", "uint16", "uint32", "uint64",
];

/// Rewrites every `{ type: integer, format: <int-format> }` schema in the spec into
/// `{ type: <int-format> }`. Our `openapi-generator.yaml` fully specifies how these are to be
/// interpreted.
pub fn lift_integer_formats(spec_yaml: &str) -> Result<String, Report> {
    let mut docs = YamlLoader::load_from_str(spec_yaml).context("Failed to parse spec YAML")?;
    assert_that!(&docs).has_length(1);
    let mut doc = docs.remove(0);

    rewrite(&mut doc);

    let mut out = String::new();
    let mut emitter = YamlEmitter::new(&mut out);
    emitter
        .dump(&doc)
        .context("Failed to emit normalized spec YAML")?;
    Ok(out)
}

fn rewrite(node: &mut Yaml) {
    match node {
        Yaml::Hash(h) => {
            for (_, v) in h.iter_mut() {
                rewrite(v);
            }
            if let Some(format) = liftable_format(h) {
                h.insert(TYPE.clone(), Yaml::String(format.to_owned()));
                h.remove(&FORMAT);
            }
        }
        Yaml::Array(arr) => {
            for v in arr.iter_mut() {
                rewrite(v);
            }
        }
        _ => {}
    }
}

fn liftable_format(h: &Hash) -> Option<&str> {
    let type_str = h.get(&TYPE)?.as_str()?;
    if type_str != "integer" {
        return None;
    }
    let format = h.get(&FORMAT)?.as_str()?;
    INT_FORMATS.contains(&format).then_some(format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn lifts_every_integer_format_and_leaves_format_less_integers_alone() {
        let input = indoc! {r#"
            components:
              schemas:
                Foo:
                  type: object
                  properties:
                    a_i8:
                      type: integer
                      format: int8
                    a_i16:
                      type: integer
                      format: int16
                    a_i32:
                      type: integer
                      format: int32
                    a_i64:
                      type: integer
                      format: int64
                    a_u8:
                      type: integer
                      format: uint8
                    a_u16:
                      type: integer
                      format: uint16
                    a_u32:
                      type: integer
                      format: uint32
                    a_u64:
                      type: integer
                      format: uint64
                    plain:
                      type: integer
        "#};
        let out = lift_integer_formats(input).unwrap();
        let expected = indoc! {r#"
            ---
            components:
              schemas:
                Foo:
                  type: object
                  properties:
                    a_i8:
                      type: int8
                    a_i16:
                      type: int16
                    a_i32:
                      type: int32
                    a_i64:
                      type: int64
                    a_u8:
                      type: uint8
                    a_u16:
                      type: uint16
                    a_u32:
                      type: uint32
                    a_u64:
                      type: uint64
                    plain:
                      type: integer"#};
        assert_that!(out).is_equal_to(expected);
    }

    #[test]
    fn lifts_array_items() {
        let input = indoc! {r#"
            components:
              schemas:
                PlatformConfig:
                  type: object
                  properties:
                    iommu_segments:
                      type: array
                      items:
                        type: integer
                        format: int16
        "#};
        let out = lift_integer_formats(input).unwrap();
        assert_that!(&out).is_equal_to(indoc! {r#"
            ---
            components:
              schemas:
                PlatformConfig:
                  type: object
                  properties:
                    iommu_segments:
                      type: array
                      items:
                        type: int16"#});
    }
}
