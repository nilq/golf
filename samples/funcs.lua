local a=function(...) local __args = {...}
if 2 == #__args then
local a = __args[1]
local b = __args[2]
return (a+b)
end

end

(a)()((1+3))()2