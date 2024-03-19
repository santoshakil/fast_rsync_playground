use std::path::PathBuf;

fn main() {
    let src = PathBuf::from("demo/src/src.txt");
    let dst = PathBuf::from("demo/dst/dst.txt");
    println!("src: {}", src.display());
    println!("dst: {}", dst.display());

    let options = fast_rsync::SignatureOptions {
        crypto_hash_size: 16,
        block_size: 1024,
    };
    let src_buf = std::fs::read(src).unwrap();
    let src_sig = fast_rsync::Signature::calculate(&src_buf, options);
    let src_index = src_sig.index();

    let dst_buf = std::fs::read(dst).unwrap();
    let dst_sig = fast_rsync::Signature::calculate(&dst_buf, options);
    // let dst_index = dst_sig.index();

    println!("src_sig: {:?}", src_sig);
    println!("dst_sig: {:?}", dst_sig);

    let mut out = vec![];
    if let Err(err) = fast_rsync::diff(&src_index, &src_buf, &mut out) {
        return eprintln!("diff error: {:?}", err);
    }
    println!("out: {:?}", out);
}
