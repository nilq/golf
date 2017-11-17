local fib=setmetatable({}, {__call = function(...)
local __args = {...}
if 2 == #__args then
if 0 == __args[2] then
return 0
end
end

if 2 == #__args then
if 1 == __args[2] then
return 1
end
end

if 2 == #__args then
local n = __args[2]
return ((fib)((n-1))+(fib)((n-2)))
end

end,
})
local twice=setmetatable({}, {__call = function(...)
local __args = {...}
if 2 == #__args then
local n = __args[2]
return (2*n)
end

end,
})
local twice_fib=function(__a) return twice(fib(__a)) end

local a=(twice_fib)(10)
