local print=setmetatable({}, {__call = function(...)
local __args = {...}
end,
})
print("yo world")
