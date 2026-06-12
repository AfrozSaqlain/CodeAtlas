
-- Seed the random number generator
math.randomseed(os.time())

-- Declaring local variables
local playerHealth = 100
local targetData = {name = "Goblin", difficulty = "Hard"}

-- Enhanced status function
local function checkStatus(health)
    if health <= 0 then
        print("💥 Player has fainted! Game Over.")
        return false
    else
        print("❤️ Player Health: " .. health)
        return true
    end
end

-- New combat function
local function takeDamage(target)
    -- Base damage varies by enemy difficulty
    local baseDamage = 10
    if target.difficulty == "Hard" then
        baseDamage = 20
    end

    -- Calculate damage variance (between 0 and 10)
    local variance = math.random(0, 10)
    local totalDamage = baseDamage + variance

    -- 20% chance for a critical hit (multiplies damage by 1.5)
    if math.random(1, 5) == 5 then
        totalDamage = math.floor(totalDamage * 1.5)
        print("⚡ CRITICAL HIT! The " .. target.name .. " strikes hard!")
    else
        print("⚔️ The " .. target.name .. " attacks!")
    end

    print("🩸 You took " .. totalDamage .. " damage.")
    return totalDamage
end

-- Simulated Game Loop
print("--- Battle Started against a " .. targetData.name .. " ---")

while playerHealth > 0 do
    -- Apply damage
    local damageTaken = takeDamage(targetData)
    playerHealth = playerHealth - damageTaken
    
    -- Check if player survives this turn
    local isAlive = checkStatus(playerHealth)
    print("-----------------------------------")
    
    -- Stop the loop if dead
    if not isAlive then
        break
    end
    
    -- Wait 1 second between rounds to simulate real-time turns
    os.execute("sleep 1" or "timeout /t 1 > nul") 
end
