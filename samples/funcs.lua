local a=setmetatable({}, {__call = function(...)
local __args = {...}
if 3 == #__args then
local a = __args[2]
local b = __args[3]
return (a+b)
end

end,
})
(a)((1+3),2)