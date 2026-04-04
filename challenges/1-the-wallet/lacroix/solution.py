# CODING COMPASS: THE WALLET
# This is a simple solution meant to demonstrate several programming concepts to those unfamiliar with them.
# Python and JSON are used for their approachability and readability. Any language and data format could be used to achieve the same result. My personal favorite combination is C# and binary.

# ===============================================================================================================
# FUNCTIONS AND DEFINITIONS
# ===============================================================================================================

import os
import json

# contains a supposed list of player names to whom the wallets are attached.
WALLETS = ["Player 1", "Player 2"]
CURRENCIES = ["Coins", "Diamonds"]

def initializeDataFile() -> None:
    """Registers wallet and currency data in a JSON file."""

    if os.path.isfile("data.json") and os.path.getsize("data.json") > 0:
        return

    data: dict[str, list[list[int]]] = {"wallets": []}

    # data is structured as a list (wallet) of lists (currency) and stored in a JSON file.
    # the index of each item corresponds to its index in the WALLETS and CURRENCIES lists.
    for _ in WALLETS:
        wallet: list[int] = []
        for _ in CURRENCIES:
            wallet.append(0)
        data["wallets"].append(wallet)

    with open("data.json", "w") as f:
        json.dump(data, f)


def updateDataFile(walletId: int, currencyId: int, operation) -> None:
    assert isValidWalletAndCurrency(walletId, currencyId)

    with open("data.json", "r+") as f:
        data = json.loads(f.read()) or {}

        assert data != {}

        data["wallets"][walletId][currencyId] = operation(
            data["wallets"][walletId][currencyId]
        )
        f.seek(0)
        json.dump(data, f)
        f.truncate()


def printData() -> None:
    """Prints wallet and currency data in a readable format."""

    with open("data.json", "r") as f:
        data = json.load(f)["wallets"]

    for i in range(len(data)):
        print(f"{WALLETS[i]}'s wallet")

        for j in range(len(data[i])):
            print(f"{data[i][j]} {CURRENCIES[j]}")

        print("")


def isValidWalletAndCurrency(walletId: int, currencyId: int) -> bool:
    return walletId < len(WALLETS) and currencyId < len(CURRENCIES)


def gainCurrency(walletId: int, currencyId: int, amount: int) -> None:
    """Add currency to a wallet.
    Args:
        walletId: The index of the wallet to gain currency.
        currencyId: The index of the currency to add.
        amount: The amount of currency to add.
    """

    assert amount > 0 and isValidWalletAndCurrency(walletId, currencyId)
    updateDataFile(walletId, currencyId, lambda x: x + amount)


def spendCurrency(walletId: int, currencyId: int, amount: int) -> bool:
    """Spend currency from a wallet.
    Args:
        walletId: The index of the wallet to spend currency from.
        currencyId: The index of the currency to spend.
        amount: The amount of currency to spend.
    Returns:
        bool: True if the spend was successful, False otherwise.
    """

    assert amount > 0 and isValidWalletAndCurrency(walletId, currencyId)

    if getBalance(walletId, currencyId) < amount:
        return False
    
    updateDataFile(walletId, currencyId, lambda x: x - amount)
    return True


def getBalance(walletId: int, currencyId: int) -> int:
    """Get the balance of a currency in a wallet.
    Args:
        walletId: The index of the wallet.
        currencyId: The index of the currency.
    Returns:
        int: The balance of the currency.
    """

    assert isValidWalletAndCurrency(walletId, currencyId)

    with open("data.json", "r") as f:
        data = json.loads(f.read()) or {}
        assert data != {}
        return data["wallets"][walletId][currencyId]


# ===============================================================================================================
# NORMAL EXECUTION
# ===============================================================================================================


initializeDataFile()
# you can run spendCurrency() and gainCurrency() here, the result is saved locally and the data is printed below
printData()
