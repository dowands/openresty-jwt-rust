const ffi = require('ffi-cross');

let lib = ffi.Library('../rust/target/release/librustjwt.dylib', {
    jwt_vaild: [ffi.types.CString, [ffi.types.CString, ffi.types.CString]]
});

let result = lib.jwt_vaild("MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA+Ug6e+v7pDgqKCww6NAWN826ve9THWqnqdhrQUtcV3Kie4JZRPX7N1Dz5n+LOv0V+tjF8rKuXAqvjvUsbluSqlLZW7lW8CNGDyWPTSPnG6y7CbK09+oI0pgt38dyzUvE7j7c99t7rx8tNquKE99FjKcDwVwm3KSqae0QQwUWBvzzRJTEsUZqUJiRgfCGVNqI/u+DpqomfE48nlddLEfrXakxbbaq0Hb6CE5BalpJvHrbkrwjyDfwHQ1ynndEILn+tIBjZa6/dsHElu4dKbvpITeIQq+rkL5VzZhGZFCuHJCECb1oUj/Z0hu4EOdsPovitAbAml5L+B3B6s67AYu8nwIDAQAB"
, "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.v-oibW8B9HQGBppk4P-jHDj-yqUgFYKu2A0GQ1aXtH4");
console.log(JSON.parse(result));
