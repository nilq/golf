local new_monster=function(...) local __args = {...}
if 1 == #__args then
local hitpoints = __args[1]
return function(...) local __args = {...}
local _self=function(...) local __args = {...}
if 2 == #__args then
local damage = __args[2]
if "hurt" == __args[1] then
local hitpoints=(hitpoints-damage)

end
end

if 1 == #__args then
if "die" == __args[1] then
return (_self)()
end
end

"hurt"
return hitpoints
end


end

end

end

