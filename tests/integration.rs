use memorysafe::{errors::parse_config, ring_buffer::RingBuffer, stack::Stack};

#[test]
fn public_api_works_together() {
    let cfg = parse_config("port = 443\nworkers = 16").unwrap();
    let mut rb = RingBuffer::with_capacity(cfg.workers as usize);
    let mut st = Stack::new();
    for i in 0..20u16 {
        rb.push_overwrite(i);
        st.push(i);
    }
    assert_eq!(rb.len(), 16);
    assert_eq!(rb.iter().next(), Some(&4));
    assert_eq!(st.into_iter().take(3).collect::<Vec<_>>(), vec![19, 18, 17]);
    assert_eq!(cfg.port, 443);
}
