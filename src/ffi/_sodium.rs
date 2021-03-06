/* automatically generated by rust-bindgen */

pub const crypto_aead_xchacha20poly1305_ietf_KEYBYTES: u32 = 32;
pub const crypto_aead_xchacha20poly1305_ietf_NPUBBYTES: u32 = 24;
pub const crypto_aead_xchacha20poly1305_ietf_ABYTES: u32 = 16;
pub const crypto_auth_hmacsha512256_BYTES: u32 = 32;
pub const crypto_auth_hmacsha512256_KEYBYTES: u32 = 32;
pub const crypto_generichash_BYTES: u32 = 32;
pub const crypto_generichash_KEYBYTES: u32 = 32;
pub const crypto_pwhash_ALG_ARGON2I13: u32 = 1;
pub const crypto_pwhash_ALG_ARGON2ID13: u32 = 2;
pub const crypto_pwhash_ALG_DEFAULT: u32 = 2;
pub const crypto_pwhash_BYTES_MIN: u32 = 16;
pub const crypto_pwhash_PASSWD_MIN: u32 = 0;
pub const crypto_pwhash_PASSWD_MAX: u32 = 4294967295;
pub const crypto_pwhash_SALTBYTES: u32 = 16;
pub const crypto_pwhash_STRBYTES: u32 = 128;
pub const crypto_pwhash_STRPREFIX: &'static [u8; 11usize] = b"$argon2id$\0";
pub const crypto_pwhash_OPSLIMIT_MIN: u32 = 1;
pub const crypto_pwhash_OPSLIMIT_MAX: u32 = 4294967295;
pub const crypto_pwhash_MEMLIMIT_MIN: u32 = 8192;
pub const crypto_pwhash_OPSLIMIT_INTERACTIVE: u32 = 2;
pub const crypto_pwhash_MEMLIMIT_INTERACTIVE: u32 = 67108864;
pub const crypto_pwhash_OPSLIMIT_MODERATE: u32 = 3;
pub const crypto_pwhash_MEMLIMIT_MODERATE: u32 = 268435456;
pub const crypto_pwhash_OPSLIMIT_SENSITIVE: u32 = 4;
pub const crypto_pwhash_MEMLIMIT_SENSITIVE: u32 = 1073741824;
pub const crypto_sign_BYTES: u32 = 64;
pub const crypto_sign_PUBLICKEYBYTES: u32 = 32;
pub const crypto_sign_SECRETKEYBYTES: u32 = 64;
extern "C" {
    pub fn sodium_init() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_encrypt(
        c: *mut ::std::os::raw::c_uchar,
        clen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        nsec: *const ::std::os::raw::c_uchar,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_decrypt(
        m: *mut ::std::os::raw::c_uchar,
        mlen_p: *mut ::std::os::raw::c_ulonglong,
        nsec: *mut ::std::os::raw::c_uchar,
        c: *const ::std::os::raw::c_uchar,
        clen: ::std::os::raw::c_ulonglong,
        ad: *const ::std::os::raw::c_uchar,
        adlen: ::std::os::raw::c_ulonglong,
        npub: *const ::std::os::raw::c_uchar,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_aead_xchacha20poly1305_ietf_keygen(k: *mut ::std::os::raw::c_uchar);
}
#[repr(C)]
pub struct crypto_hash_sha512_state {
    pub state: [u64; 8usize],
    pub count: [u64; 2usize],
    pub buf: [u8; 128usize],
}
#[test]
fn bindgen_test_layout_crypto_hash_sha512_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_hash_sha512_state>(),
        208usize,
        concat!("Size of: ", stringify!(crypto_hash_sha512_state))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_hash_sha512_state>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_hash_sha512_state))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha512_state>())).state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha512_state),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha512_state>())).count as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha512_state),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_hash_sha512_state>())).buf as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_hash_sha512_state),
            "::",
            stringify!(buf)
        )
    );
}
#[repr(C)]
pub struct crypto_auth_hmacsha512_state {
    pub ictx: crypto_hash_sha512_state,
    pub octx: crypto_hash_sha512_state,
}
#[test]
fn bindgen_test_layout_crypto_auth_hmacsha512_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_auth_hmacsha512_state>(),
        416usize,
        concat!("Size of: ", stringify!(crypto_auth_hmacsha512_state))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_auth_hmacsha512_state>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_auth_hmacsha512_state))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_auth_hmacsha512_state>())).ictx as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_auth_hmacsha512_state),
            "::",
            stringify!(ictx)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_auth_hmacsha512_state>())).octx as *const _ as usize
        },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_auth_hmacsha512_state),
            "::",
            stringify!(octx)
        )
    );
}
extern "C" {
    pub fn crypto_auth_hmacsha512256(
        out: *mut ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_verify(
        h: *const ::std::os::raw::c_uchar,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        k: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
pub type crypto_auth_hmacsha512256_state = crypto_auth_hmacsha512_state;
extern "C" {
    pub fn crypto_auth_hmacsha512256_init(
        state: *mut crypto_auth_hmacsha512256_state,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_update(
        state: *mut crypto_auth_hmacsha512256_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_final(
        state: *mut crypto_auth_hmacsha512256_state,
        out: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_auth_hmacsha512256_keygen(k: *mut ::std::os::raw::c_uchar);
}
#[repr(C)]
pub struct crypto_generichash_blake2b_state {
    pub opaque: [::std::os::raw::c_uchar; 384usize],
}
#[test]
fn bindgen_test_layout_crypto_generichash_blake2b_state() {
    assert_eq!(
        ::std::mem::size_of::<crypto_generichash_blake2b_state>(),
        384usize,
        concat!("Size of: ", stringify!(crypto_generichash_blake2b_state))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<crypto_generichash_blake2b_state>())).opaque as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_generichash_blake2b_state),
            "::",
            stringify!(opaque)
        )
    );
}
pub type crypto_generichash_state = crypto_generichash_blake2b_state;
extern "C" {
    pub fn crypto_generichash_statebytes() -> usize;
}
extern "C" {
    pub fn crypto_generichash(
        out: *mut ::std::os::raw::c_uchar,
        outlen: usize,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_generichash_init(
        state: *mut crypto_generichash_state,
        key: *const ::std::os::raw::c_uchar,
        keylen: usize,
        outlen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_generichash_update(
        state: *mut crypto_generichash_state,
        in_: *const ::std::os::raw::c_uchar,
        inlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_generichash_final(
        state: *mut crypto_generichash_state,
        out: *mut ::std::os::raw::c_uchar,
        outlen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_generichash_keygen(k: *mut ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn crypto_pwhash_str(
        out: *mut ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_str_verify(
        str: *const ::std::os::raw::c_char,
        passwd: *const ::std::os::raw::c_char,
        passwdlen: ::std::os::raw::c_ulonglong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_pwhash_str_needs_rehash(
        str: *const ::std::os::raw::c_char,
        opslimit: ::std::os::raw::c_ulonglong,
        memlimit: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_keypair(
        pk: *mut ::std::os::raw::c_uchar,
        sk: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_detached(
        sig: *mut ::std::os::raw::c_uchar,
        siglen_p: *mut ::std::os::raw::c_ulonglong,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        sk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_verify_detached(
        sig: *const ::std::os::raw::c_uchar,
        m: *const ::std::os::raw::c_uchar,
        mlen: ::std::os::raw::c_ulonglong,
        pk: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn randombytes_buf(buf: *mut ::std::os::raw::c_void, size: usize);
}
extern "C" {
    pub fn sodium_memzero(pnt: *mut ::std::os::raw::c_void, len: usize);
}
extern "C" {
    pub fn sodium_memcmp(
        b1_: *const ::std::os::raw::c_void,
        b2_: *const ::std::os::raw::c_void,
        len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_pad(
        padded_buflen_p: *mut usize,
        buf: *mut ::std::os::raw::c_uchar,
        unpadded_buflen: usize,
        blocksize: usize,
        max_buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sodium_unpad(
        unpadded_buflen_p: *mut usize,
        buf: *const ::std::os::raw::c_uchar,
        padded_buflen: usize,
        blocksize: usize,
    ) -> ::std::os::raw::c_int;
}
