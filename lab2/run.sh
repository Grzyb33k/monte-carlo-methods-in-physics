#!/bin/bash

# Kolory dla lepszej czytelności w terminalu
GREEN='\033[0-32m'
BLUE='\033[0-34m'
NC='\033[0m' # No Color

echo -e "${BLUE}== Rozpoczynam laboratorium: Generatory liczb pseudolosowych 1D ==${NC}"

# 1. Kompilacja i uruchomienie Rusta w trybie release
# To kluczowe dla wydajności przy N=10^7 
echo -e "${GREEN}>> Krok 1: Generowanie danych (Rust)...${NC}"
cargo run --release

if [ $? -ne 0 ]; then
    echo "Błąd podczas wykonywania programu w Ruście!"
    exit 1
fi

# 2. Uruchomienie skryptu Pythona do wykresów
# Skrypt przetworzy pliki CSV dla p=0.1, 0.5, 0.9 
echo -e "${GREEN}>> Krok 2: Tworzenie wykresów (Python)...${NC}"
python3 scripts/make_plots.py

if [ $? -ne 0 ]; then
    echo "Błąd podczas generowania wykresów!"
    exit 1
fi

echo -e "${BLUE}== Sukces! Wyniki znajdziesz w folderach /results i /plots ==${NC}"