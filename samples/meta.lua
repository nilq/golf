local vec2=setmetatable({}, {__call = function(...)
local __args = {...}
if 3 == #__args then
local x = __args[2]
local y = __args[3]
return setmetatable({}, {__add = function(_, v)
return (vec2)((x+(v)("x")),(y+(v)("y")))
end,
__call = function(...)
local __args = {...}
if 2 == #__args then
if "x" == __args[2] then
return x
end
end

if 2 == #__args then
if "y" == __args[2] then
return y
end
end

if 3 == #__args then
local v = __args[3]
return (vec2)((x+(v)("x")),(y+(v)("y")))
end

end,
})
end

end,
})
local a=(vec2)(100,100)
local b=(vec2)(200,200)
local c=(a+b)
