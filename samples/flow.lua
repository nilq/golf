local print=1
local _if=function(...) local __args = {...}
if 2 == #__args then
local body = __args[2]
if true == __args[1] then
return (body)()
end
end

end

local range=function(...) local __args = {...}
if 3 == #__args then
local a = __args[1]
local b = __args[2]
local body = __args[3]
return (_if)((a<b))
end

return function(...) local __args = {...}
(body)()
return (range)(((a+1))((b)(body)))
end

end

(range)(1)10function(...) local __args = {...}
return (print)("yoyo")
end
