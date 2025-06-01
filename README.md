# TASK

## Zbuduj

``` bash
anchor build
```

## IDL

Znajdz w target folder `idl` z *.json* zawierający idl smart contact

Plik potrzeba zaimportować do frontendu by stworzyć obiekt program z odpowiednimi metodami

## Stwórz frontend

1. Logowanie Walletem obsługującym nft - Phantom

```
https://medium.com/@jorge_londono_31005/intro-to-solana-webapp-development-with-react-typescript-phantom-ca2724d1fa22
```

Użyj biblioteki '@solana/web3.js' oraz znajdź w dokumentacji jak zaimplementować wallet provider potrzebny do odwoływania się do kontekstu wallet w client side

2. Form dla IDL

Stwórz przy pomocy 'coral-xyz/anchor' program na podstawie wcześniej pobranego idl.

```
https://www.anchor-lang.com/docs/clients/typescript
```

3. Przycisk wywołujący interacje przez wallet

Należy stworzyc transakcje wywołująca montująca instrukcje z programu, a w niej podać odpowiednie parametry - sprawdź zawartość idl lub lib.rs

ZROBIONY ZOSTAŁ DEPLOY NA DEVNET NA `AP9Q9Zni19LTXPr1yvrv93kAVZfjnBETNey3VspK4GSh`

Zalecam console.log oraz try catch by poprawnie obsługiwać błędy

Jeśli jest wszystko poprawne to w portfele na sieci Devnet pojawi się nft z image

stworz fork - oddanie ma zostać przez pull request z udaną transakcją nft z Devnet

zadanie z gwiazką 

1. Stworzenie upload img z frontend zamiast url

2. Dodać własną funkcjonalność - nic nie podane w celu własnej kreatywności z uwagi bycia zadaniem dodatkowym
