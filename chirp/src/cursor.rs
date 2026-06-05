struct BufferCursor<'d, 's> {
    buffer: &'d mut [f32],
    resume: Option<&'s [f32]>,
}
