use mlt_rs::profile::Profile;
use mlt_rs::xml::{MltDocument, MltXmlProducer, MltXmlProperty};

#[test]
fn test_mlt_xml_roundtrip() -> anyhow::Result<()> {
    let profile = Profile::hd_1080_30p();
    let mut doc = MltDocument::new(&profile);

    doc.producers.push(MltXmlProducer {
        id: "prod0".to_string(),
        in_point: 0,
        out_point: 89,
        properties: vec![
            MltXmlProperty {
                name: "resource".to_string(),
                value: "interview.mp4".to_string(),
            },
        ],
    });

    let xml = doc.to_xml()?;
    assert!(xml.contains("<mlt"));
    assert!(xml.contains("interview.mp4"));
    assert!(xml.contains("1920"));
    assert!(xml.contains("1080"));

    let parsed = MltDocument::from_xml(&xml)?;
    assert_eq!(parsed.profile.width, 1920);
    assert_eq!(parsed.profile.height, 1080);
    assert_eq!(parsed.producers.len(), 1);
    assert_eq!(parsed.producers[0].properties[0].value, "interview.mp4");

    Ok(())
}
