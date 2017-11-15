local fib=function(...) local __args = {...}
if 1 == #__args then
if 0 == __args[1] then
return 0
end
end

local a=10

if 1 == #__args then
if 1 == __args[1] then
return 1
end
end

if 1 == #__args then
local n = __args[1]
return ((fib)((n-1))+(fib)((n-2)))
end

end

local twice=function(...) local __args = {...}
if 1 == #__args then
local n = __args[1]
return (2*n)
end

end

local twice_fib=function(__a) return twice(fib(__a)) end

local a=(twice_fib)(10)
