local cjson = require "cjson"

local key = "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA+Ug6e+v7pDgqKCww6NAWN826ve9THWqnqdhrQUtcV3Kie4JZRPX7N1Dz5n+LOv0V+tjF8rKuXAqvjvUsbluSqlLZW7lW8CNGDyWPTSPnG6y7CbK09+oI0pgt38dyzUvE7j7c99t7rx8tNquKE99FjKcDwVwm3KSqae0QQwUWBvzzRJTEsUZqUJiRgfCGVNqI/u+DpqomfE48nlddLEfrXakxbbaq0Hb6CE5BalpJvHrbkrwjyDfwHQ1ynndEILn+tIBjZa6/dsHElu4dKbvpITeIQq+rkL5VzZhGZFCuHJCECb1oUj/Z0hu4EOdsPovitAbAml5L+B3B6s67AYu8nwIDAQAB"
local headers = ngx.req.get_headers()

if headers["Authorization"] == nil then
    return 
end

local ffi = require("ffi")

ffi.cdef[[
    const char * jwt_vaild(const char *key, const char *auth);
]]

local C = ffi.load("/etc/nginx/lua/librustjwt.so")
local jsonStr = ffi.string(C.jwt_vaild(key, headers["Authorization"]))
if string.sub(jsonStr, 1, 5) == "ERROR" then
    ngx.say(jsonStr)
    ngx.exit(500)
    return
end

local jwtObject = cjson.decode(jsonStr)

if jwtObject ~= nil and jwtObject.user ~= nil then
    local userObject = cjson.decode(jwtObject.user)
    if userObject ~= nil and userObject.unionId ~= nil then
        ngx.req.set_header("user", jwtObject.user)
        ngx.req.set_header("unionId", userObject.unionId)
    else
        ngx.log(ngx.WARN, cjson.encode(jwtObject))
    end
else
    ngx.say("jwt error")
    ngx.exit(500)
end

