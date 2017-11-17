local new_monster=setmetatable({}, {__call = function(...)
local __args = {...}
if 2 == #__args then
local hitpoints = __args[2]
return setmetatable({}, {__call = function(...)
local __args = {...}
local _self=setmetatable({}, {__call = function(...)
local __args = {...}
if 3 == #__args then
local damage = __args[3]
if "hurt" == __args[2] then
local hitpoints=(hitpoints-damage)

end
end

if 2 == #__args then
if "die" == __args[2] then
return (_self)("hurt",hitpoints)
end
end

end,
})

end,
})
end

end,
})
