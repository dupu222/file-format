use file_format::FileFormat;

#[test]
fn test_aac() {
    let format = FileFormat::from_file("fixtures/audio/sample.aac").unwrap();
    assert_eq!(format, FileFormat::Aac);
    assert_eq!(format.media_type(), "audio/aac");
    assert_eq!(format.preferred_extension(), "aac");
}

#[test]
fn test_ac3() {
    let format = FileFormat::from_file("fixtures/audio/sample.ac3").unwrap();
    assert_eq!(format, FileFormat::Ac3);
    assert_eq!(format.media_type(), "audio/vnd.dolby.dd-raw");
    assert_eq!(format.preferred_extension(), "ac3");
}

#[test]
fn test_aif() {
    let format = FileFormat::from_file("fixtures/audio/sample.aif").unwrap();
    assert_eq!(format, FileFormat::Aiff);
    assert_eq!(format.media_type(), "audio/aiff");
    assert_eq!(format.preferred_extension(), "aif");
}

#[test]
fn test_amr() {
    let format = FileFormat::from_file("fixtures/audio/sample.amr").unwrap();
    assert_eq!(format, FileFormat::Amr);
    assert_eq!(format.media_type(), "audio/amr");
    assert_eq!(format.preferred_extension(), "amr");
}

#[test]
fn test_ape() {
    let format = FileFormat::from_file("fixtures/audio/sample.ape").unwrap();
    assert_eq!(format, FileFormat::Ape);
    assert_eq!(format.media_type(), "audio/x-ape");
    assert_eq!(format.preferred_extension(), "ape");
}

#[test]
fn test_au() {
    let format = FileFormat::from_file("fixtures/audio/sample.au").unwrap();
    assert_eq!(format, FileFormat::Snd);
    assert_eq!(format.media_type(), "audio/basic");
    assert_eq!(format.preferred_extension(), "au");
}

#[test]
fn test_flac() {
    let format = FileFormat::from_file("fixtures/audio/sample.flac").unwrap();
    assert_eq!(format, FileFormat::Flac);
    assert_eq!(format.media_type(), "audio/x-flac");
    assert_eq!(format.preferred_extension(), "flac");
}

#[test]
fn test_m4a() {
    let format = FileFormat::from_file("fixtures/audio/sample.m4a").unwrap();
    assert_eq!(format, FileFormat::M4a);
    assert_eq!(format.media_type(), "audio/x-m4a");
    assert_eq!(format.preferred_extension(), "m4a");
}

#[test]
fn test_mid() {
    let format = FileFormat::from_file("fixtures/audio/sample.mid").unwrap();
    assert_eq!(format, FileFormat::Midi);
    assert_eq!(format.media_type(), "audio/midi");
    assert_eq!(format.preferred_extension(), "mid");
}

#[test]
fn test_mp3() {
    let format = FileFormat::from_file("fixtures/audio/sample.mp3").unwrap();
    assert_eq!(format, FileFormat::Mp3);
    assert_eq!(format.media_type(), "audio/mpeg");
    assert_eq!(format.preferred_extension(), "mp3");
}

#[test]
fn test_mpc() {
    let format = FileFormat::from_file("fixtures/audio/sample.mpc").unwrap();
    assert_eq!(format, FileFormat::Mpc);
    assert_eq!(format.media_type(), "audio/x-musepack");
    assert_eq!(format.preferred_extension(), "mpc");
}

#[test]
fn test_ogg() {
    let format = FileFormat::from_file("fixtures/audio/sample.ogg").unwrap();
    assert_eq!(format, FileFormat::OggAudio);
    assert_eq!(format.media_type(), "audio/ogg");
    assert_eq!(format.preferred_extension(), "ogg");
}

#[test]
fn test_wav() {
    let format = FileFormat::from_file("fixtures/audio/sample.wav").unwrap();
    assert_eq!(format, FileFormat::Wave);
    assert_eq!(format.media_type(), "audio/vnd.wave");
    assert_eq!(format.preferred_extension(), "wav");
}

#[test]
fn test_wv() {
    let format = FileFormat::from_file("fixtures/audio/sample.wv").unwrap();
    assert_eq!(format, FileFormat::WavPack);
    assert_eq!(format.media_type(), "audio/wavpack");
    assert_eq!(format.preferred_extension(), "wv");
}
