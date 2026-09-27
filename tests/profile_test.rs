use mlt_rs::profile::Profile;

#[test]
fn test_broadcast_profiles() {
    let p_1080p = Profile::hd_1080_30p();
    assert_eq!(p_1080p.width, 1920);
    assert_eq!(p_1080p.height, 1080);
    assert_eq!(p_1080p.fps(), 30.0);
    assert!((p_1080p.dar() - 16.0 / 9.0).abs() < 1e-4);

    let p_4k = Profile::uhd_2160_60p();
    assert_eq!(p_4k.width, 3840);
    assert_eq!(p_4k.height, 2160);
    assert_eq!(p_4k.fps(), 60.0);

    let p_vert = Profile::vertical_hd_30p();
    assert_eq!(p_vert.width, 1080);
    assert_eq!(p_vert.height, 1920);
    assert!((p_vert.dar() - 9.0 / 16.0).abs() < 1e-4);
}

#[test]
fn test_profile_from_name() {
    assert!(Profile::from_name("1080p30").is_some());
    assert!(Profile::from_name("4k60").is_some());
    assert!(Profile::from_name("tiktok").is_some());
    assert!(Profile::from_name("unknown_invalid_profile").is_none());
}
