/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/pump.json`.
 */
export type Pump = {
  "address": "7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5",
  "metadata": {
    "name": "pump",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "configure",
      "discriminator": [
        245,
        7,
        108,
        117,
        95,
        196,
        54,
        217
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "globalConfig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "newConfig",
          "type": {
            "defined": {
              "name": "config"
            }
          }
        }
      ]
    },
    {
      "name": "launch",
      "discriminator": [
        153,
        241,
        93,
        225,
        22,
        69,
        74,
        61
      ],
      "accounts": [
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "globalConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "tokenMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "bondingCurve",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  110,
                  100,
                  105,
                  110,
                  103,
                  45,
                  99,
                  117,
                  114,
                  118,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "curveTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "bondingCurve"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "tokenMetadataAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "metadataProgram",
          "address": "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "symbol",
          "type": "string"
        },
        {
          "name": "uri",
          "type": "string"
        }
      ]
    },
    {
      "name": "migrate",
      "discriminator": [
        155,
        234,
        231,
        146,
        236,
        158,
        162,
        30
      ],
      "accounts": [
        {
          "name": "ammProgram",
          "address": "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"
        },
        {
          "name": "amm",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  231,
                  175,
                  132,
                  116,
                  99,
                  232,
                  61,
                  1,
                  17,
                  152,
                  120,
                  186,
                  94,
                  24,
                  164,
                  143,
                  42,
                  111,
                  154,
                  213,
                  38,
                  209,
                  30,
                  51,
                  92,
                  129,
                  56,
                  1,
                  25,
                  175,
                  218,
                  253
                ]
              },
              {
                "kind": "account",
                "path": "market"
              },
              {
                "kind": "const",
                "value": [
                  97,
                  109,
                  109,
                  95,
                  97,
                  115,
                  115,
                  111,
                  99,
                  105,
                  97,
                  116,
                  101,
                  100,
                  95,
                  115,
                  101,
                  101,
                  100
                ]
              }
            ]
          }
        },
        {
          "name": "ammAuthority",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  109,
                  109,
                  32,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "ammOpenOrders",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  231,
                  175,
                  132,
                  116,
                  99,
                  232,
                  61,
                  1,
                  17,
                  152,
                  120,
                  186,
                  94,
                  24,
                  164,
                  143,
                  42,
                  111,
                  154,
                  213,
                  38,
                  209,
                  30,
                  51,
                  92,
                  129,
                  56,
                  1,
                  25,
                  175,
                  218,
                  253
                ]
              },
              {
                "kind": "account",
                "path": "market"
              },
              {
                "kind": "const",
                "value": [
                  111,
                  112,
                  101,
                  110,
                  95,
                  111,
                  114,
                  100,
                  101,
                  114,
                  95,
                  97,
                  115,
                  115,
                  111,
                  99,
                  105,
                  97,
                  116,
                  101,
                  100,
                  95,
                  115,
                  101,
                  101,
                  100
                ]
              }
            ]
          }
        },
        {
          "name": "ammLpMint",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  231,
                  175,
                  132,
                  116,
                  99,
                  232,
                  61,
                  1,
                  17,
                  152,
                  120,
                  186,
                  94,
                  24,
                  164,
                  143,
                  42,
                  111,
                  154,
                  213,
                  38,
                  209,
                  30,
                  51,
                  92,
                  129,
                  56,
                  1,
                  25,
                  175,
                  218,
                  253
                ]
              },
              {
                "kind": "account",
                "path": "market"
              },
              {
                "kind": "const",
                "value": [
                  108,
                  112,
                  95,
                  109,
                  105,
                  110,
                  116,
                  95,
                  97,
                  115,
                  115,
                  111,
                  99,
                  105,
                  97,
                  116,
                  101,
                  100,
                  95,
                  115,
                  101,
                  101,
                  100
                ]
              }
            ]
          }
        },
        {
          "name": "ammCoinMint"
        },
        {
          "name": "ammPcMint"
        },
        {
          "name": "ammCoinVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  231,
                  175,
                  132,
                  116,
                  99,
                  232,
                  61,
                  1,
                  17,
                  152,
                  120,
                  186,
                  94,
                  24,
                  164,
                  143,
                  42,
                  111,
                  154,
                  213,
                  38,
                  209,
                  30,
                  51,
                  92,
                  129,
                  56,
                  1,
                  25,
                  175,
                  218,
                  253
                ]
              },
              {
                "kind": "account",
                "path": "market"
              },
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  105,
                  110,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116,
                  95,
                  97,
                  115,
                  115,
                  111,
                  99,
                  105,
                  97,
                  116,
                  101,
                  100,
                  95,
                  115,
                  101,
                  101,
                  100
                ]
              }
            ]
          }
        },
        {
          "name": "ammPcVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  231,
                  175,
                  132,
                  116,
                  99,
                  232,
                  61,
                  1,
                  17,
                  152,
                  120,
                  186,
                  94,
                  24,
                  164,
                  143,
                  42,
                  111,
                  154,
                  213,
                  38,
                  209,
                  30,
                  51,
                  92,
                  129,
                  56,
                  1,
                  25,
                  175,
                  218,
                  253
                ]
              },
              {
                "kind": "account",
                "path": "market"
              },
              {
                "kind": "const",
                "value": [
                  112,
                  99,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116,
                  95,
                  97,
                  115,
                  115,
                  111,
                  99,
                  105,
                  97,
                  116,
                  101,
                  100,
                  95,
                  115,
                  101,
                  101,
                  100
                ]
              }
            ]
          }
        },
        {
          "name": "ammTargetOrders",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  231,
                  175,
                  132,
                  116,
                  99,
                  232,
                  61,
                  1,
                  17,
                  152,
                  120,
                  186,
                  94,
                  24,
                  164,
                  143,
                  42,
                  111,
                  154,
                  213,
                  38,
                  209,
                  30,
                  51,
                  92,
                  129,
                  56,
                  1,
                  25,
                  175,
                  218,
                  253
                ]
              },
              {
                "kind": "account",
                "path": "market"
              },
              {
                "kind": "const",
                "value": [
                  116,
                  97,
                  114,
                  103,
                  101,
                  116,
                  95,
                  97,
                  115,
                  115,
                  111,
                  99,
                  105,
                  97,
                  116,
                  101,
                  100,
                  95,
                  115,
                  101,
                  101,
                  100
                ]
              }
            ]
          }
        },
        {
          "name": "ammConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  109,
                  109,
                  95,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103,
                  95,
                  97,
                  99,
                  99,
                  111,
                  117,
                  110,
                  116,
                  95,
                  115,
                  101,
                  101,
                  100
                ]
              }
            ]
          }
        },
        {
          "name": "createFeeDestination",
          "writable": true,
          "address": "7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5"
        },
        {
          "name": "marketProgram",
          "address": "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX"
        },
        {
          "name": "market"
        },
        {
          "name": "userWallet",
          "writable": true,
          "signer": true
        },
        {
          "name": "userTokenCoin",
          "writable": true
        },
        {
          "name": "userTokenPc",
          "writable": true
        },
        {
          "name": "userTokenLp",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "userWallet"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "ammLpMint"
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "sysvarRent",
          "address": "SysvarRent111111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "nonce",
          "type": "u8"
        },
        {
          "name": "openTime",
          "type": "u64"
        },
        {
          "name": "initPcAmount",
          "type": "u64"
        },
        {
          "name": "initCoinAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "swap",
      "discriminator": [
        248,
        198,
        158,
        145,
        225,
        117,
        135,
        200
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "globalConfig",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "feeRecipient",
          "writable": true
        },
        {
          "name": "bondingCurve",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  111,
                  110,
                  100,
                  105,
                  110,
                  103,
                  45,
                  99,
                  117,
                  114,
                  118,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ]
          }
        },
        {
          "name": "tokenMint"
        },
        {
          "name": "curveTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "bondingCurve"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "userTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "user"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "tokenMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "direction",
          "type": "u8"
        },
        {
          "name": "minOut",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "bondingCurve",
      "discriminator": [
        23,
        183,
        248,
        55,
        96,
        216,
        172,
        96
      ]
    },
    {
      "name": "config",
      "discriminator": [
        155,
        12,
        170,
        224,
        30,
        250,
        204,
        130
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "notAuthorized",
      "msg": "Not authorized address"
    },
    {
      "code": 6001,
      "name": "incorrectFeeRecipient",
      "msg": "Fee recipient address is not match with the one in the config"
    },
    {
      "code": 6002,
      "name": "incorrectValue",
      "msg": "The value is not in the expected range"
    },
    {
      "code": 6003,
      "name": "returnAmountTooSmall",
      "msg": "Amount out is smaller than required amount"
    },
    {
      "code": 6004,
      "name": "overflowOrUnderflowOccurred",
      "msg": "An overflow or underflow occurred during the calculation"
    },
    {
      "code": 6005,
      "name": "curveAlreadyCompleted",
      "msg": "Curve is already completed"
    }
  ],
  "types": [
    {
      "name": "bondingCurve",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "virtualTokenReserves",
            "type": "u64"
          },
          {
            "name": "virtualSolReserves",
            "type": "u64"
          },
          {
            "name": "realTokenReserves",
            "type": "u64"
          },
          {
            "name": "realSolReserves",
            "type": "u64"
          },
          {
            "name": "tokenTotalSupply",
            "type": "u64"
          },
          {
            "name": "isCompleted",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "config",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "feeRecipient",
            "type": "pubkey"
          },
          {
            "name": "curveLimit",
            "type": "u64"
          },
          {
            "name": "initialVirtualTokenReserves",
            "type": "u64"
          },
          {
            "name": "initialVirtualSolReserves",
            "type": "u64"
          },
          {
            "name": "initialRealTokenReserves",
            "type": "u64"
          },
          {
            "name": "totalTokenSupply",
            "type": "u64"
          },
          {
            "name": "buyFeePercent",
            "type": "f64"
          },
          {
            "name": "sellFeePercent",
            "type": "f64"
          },
          {
            "name": "migrationFeePercent",
            "type": "f64"
          }
        ]
      }
    }
  ]
};
