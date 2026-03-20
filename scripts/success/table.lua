local M = {}
---@param x number
---@return number
M.abs = function(x)
	if x < 0 then
		return -x
	else
		return x * 2
	end
end
return M
