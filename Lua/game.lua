-- ==========================================
-- Guess the Number RPG
-- ==========================================

math.randomseed(os.time())

-------------------------------------------------
-- Helper Functions
-------------------------------------------------

local function drawHealthBar(current, maximum, length)
    length = length or 20
    local ratio = math.max(current, 0) / maximum
    local filled = math.floor(ratio * length)
    local empty = length - filled

    return "[" ..
        string.rep("█", filled) ..
        string.rep("░", empty) ..
        "] " ..
        current .. "/" .. maximum
end

local function printStatus(playerHealth, playerMax, goblinHealth, goblinMax)
    print("\n==============================================")
    print("🧑 Player : " .. drawHealthBar(playerHealth, playerMax))
    print("👹 Goblin : " .. drawHealthBar(goblinHealth, goblinMax))
    print("==============================================\n")
end

-------------------------------------------------
-- Difficulty Selection
-------------------------------------------------

print("========== Guess the Number RPG ==========\n")

io.write("Choose the maximum possible number (Example: 50, 100, 500): ")
local maxNumber = tonumber(io.read())

if not maxNumber or maxNumber < 20 then
    maxNumber = 100
end

print("\nThe game will generate a random number between 1 and " .. maxNumber)
print("If your guess is within ±10 of the hidden number,")
print("you hit the Goblin!")
print("Otherwise the Goblin attacks you.\n")

-------------------------------------------------
-- Player & Enemy
-------------------------------------------------

local player = {
    health = 100,
    maxHealth = 100
}

local goblin = {
    name = "Goblin",
    health = 120,
    maxHealth = 120
}

-------------------------------------------------
-- Combat Functions
-------------------------------------------------

local function playerAttack()
    local damage = math.random(18, 35)

    if math.random(1,5) == 5 then
        damage = math.floor(damage * 1.5)
        print("\n✨ CRITICAL STRIKE!")
    end

    goblin.health = math.max(0, goblin.health - damage)

    print("🗡️  You dealt " .. damage .. " damage!")
end

local function goblinAttack()
    local damage = math.random(12, 28)

    if math.random(1,5) == 5 then
        damage = math.floor(damage * 1.5)
        print("\n⚡ The Goblin lands a CRITICAL HIT!")
    end

    player.health = math.max(0, player.health - damage)

    print("👹 Goblin dealt " .. damage .. " damage!")
end

-------------------------------------------------
-- Main Game Loop
-------------------------------------------------

while player.health > 0 and goblin.health > 0 do

    printStatus(
        player.health,
        player.maxHealth,
        goblin.health,
        goblin.maxHealth
    )

    local targetNumber = math.random(1, maxNumber)

    io.write("🎯 Enter your guess: ")
    local guess = tonumber(io.read())

    if not guess then
        print("Invalid input! Turn wasted.")
        goblinAttack()
    else
        local errorValue = math.abs(guess - targetNumber)

        print("\n🎲 Hidden Number : " .. targetNumber)
        print("📏 Error         : " .. errorValue)

        if errorValue <= 10 then
            print("\n✅ Excellent guess!")
            playerAttack()
        else
            print("\n❌ Too far away!")
            goblinAttack()
        end
    end

    if goblin.health <= 0 then
        break
    end

    if player.health <= 0 then
        break
    end

    print("\nPress Enter for next round...")
    io.read()
end

-------------------------------------------------
-- Game Result
-------------------------------------------------

printStatus(
    player.health,
    player.maxHealth,
    goblin.health,
    goblin.maxHealth
)

if player.health <= 0 then
    print("💀 You were defeated!")
elseif goblin.health <= 0 then
    print("🏆 Victory! The Goblin has been slain!")
end

print("\n========== Game Over ==========")
