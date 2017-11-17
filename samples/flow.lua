local print=1
local _if=setmetatable({}, {__call = function(...)
local __args = {...}
if 3 == #__args then
local body = __args[3]
if true == __args[2] then
return (body)()
end
end

end,
})
local range=setmetatable({}, {__call = function(...)
local __args = {...}
if 4 == #__args then
local a = __args[2]
local b = __args[3]
local body = __args[4]
return (_if)((a<b),setmetatable({}, {__call = function(...)
local __args = {...}
(body)()
return (range)((a+1),b,body)
end,
}))
end

end,
})
(range)(1,10,setmetatable({}, {__call = function(...)
local __args = {...}
return (print)("yoyo")
end,
}))