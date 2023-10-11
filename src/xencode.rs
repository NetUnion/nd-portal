use crate::base64::base64_encode;

fn u8_array_to_u32_array(content: &[u8]) -> Vec<u32> {
    let chunks = content.chunks(4);
    chunks
        .map(|chunk| u32::from_le_bytes([chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0), *chunk.get(3).unwrap_or(&0)]))
        .collect()
}

fn u32_array_to_u8_array(content: &[u32]) -> Vec<u8> {
    let mut result = Vec::with_capacity(content.len() * 4);
    for &i in content {
        result.extend_from_slice(&i.to_le_bytes());
    }
    result
}

fn xencode(message: &mut Vec<u32>, key: &[u32]) {
    let n: u32 = (message.len() - 1).try_into().unwrap();
    let mut z: u32 = message[n as usize];
    let mut y;
    let c: u32 = 0x9e3779b9;
    let mut m;
    let mut e;
    let mut p: u32;
    let mut q = 6 + 52 / (n + 1);
    let mut d: u32 = 0;
    while 0 < q {
        d = d.wrapping_add(c);
        e = d >> 2 & 3;
        p = 0;
        while p < n {
            y = message[p as usize + 1];
            m = z >> 5 ^ y << 2;
            m = m.wrapping_add((y >> 3 ^ z << 4) ^ (d ^ y));
            m = m.wrapping_add(key[(p as usize & 3) ^ e as usize] ^ z);
            message[p as usize] = message[p as usize].wrapping_add(m);
            z = message[p as usize];
            p = p + 1;
        }
        y = message[0];
        m = z >> 5 ^ y << 2;
        m = m.wrapping_add((y >> 3 ^ z << 4) ^ (d ^ y));
        m = m.wrapping_add(key[(p as usize & 3) ^ e as usize] ^ z);
        message[n as usize] = message[n as usize].wrapping_add(m);
        z = message[n as usize];
        q = q - 1;
    }
}

pub(crate) fn get_xencode(message: &str, key: &str) -> String {
    // convert message and key to u8 arrays
    let message = message.as_bytes();
    let key = key.as_bytes();

    // convert message and key to u32 arrays
    let mut message = {
        let message_len = message.len();
        let mut r = u8_array_to_u32_array(message);
        r.push(message_len as u32);
        r
    };

    let key = {
        let mut r = u8_array_to_u32_array(key);
        if r.len() < 4 {
            r.resize(4, 0);
        }
        r
    };

    xencode(&mut message, &key);

    let message = u32_array_to_u8_array(&message);
    base64_encode(&message)
}
