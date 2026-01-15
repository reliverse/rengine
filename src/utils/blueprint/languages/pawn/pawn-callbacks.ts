/**
 * SAMP callback definitions for Pawn
 */

export interface SampCallback {
  name: string;
  description: string;
  parameters: SampCallbackParameter[];
  returnType: "int" | "void";
}

export interface SampCallbackParameter {
  name: string;
  type: string;
  description?: string;
}

export const SAMP_CALLBACKS: SampCallback[] = [
  {
    name: "OnGameModeInit",
    description: "Called when the game mode initializes",
    parameters: [],
    returnType: "int",
  },
  {
    name: "OnGameModeExit",
    description: "Called when the game mode exits",
    parameters: [],
    returnType: "int",
  },
  {
    name: "OnPlayerConnect",
    description: "Called when a player connects to the server",
    parameters: [
      {
        name: "playerid",
        type: "int",
        description: "The ID of the player who connected",
      },
    ],
    returnType: "int",
  },
  {
    name: "OnPlayerDisconnect",
    description: "Called when a player disconnects from the server",
    parameters: [
      {
        name: "playerid",
        type: "int",
        description: "The ID of the player who disconnected",
      },
      {
        name: "reason",
        type: "int",
        description: "The reason for disconnection",
      },
    ],
    returnType: "int",
  },
  {
    name: "OnPlayerSpawn",
    description: "Called when a player spawns",
    parameters: [
      {
        name: "playerid",
        type: "int",
        description: "The ID of the player who spawned",
      },
    ],
    returnType: "int",
  },
  {
    name: "OnPlayerDeath",
    description: "Called when a player dies",
    parameters: [
      {
        name: "playerid",
        type: "int",
        description: "The ID of the player who died",
      },
      {
        name: "killerid",
        type: "int",
        description: "The ID of the player who killed them",
      },
      { name: "reason", type: "int", description: "The reason for death" },
    ],
    returnType: "int",
  },
  {
    name: "OnPlayerCommandText",
    description: "Called when a player types a command",
    parameters: [
      { name: "playerid", type: "int", description: "The ID of the player" },
      { name: "cmdtext", type: "String", description: "The command text" },
    ],
    returnType: "int",
  },
  {
    name: "OnPlayerText",
    description: "Called when a player sends a chat message",
    parameters: [
      { name: "playerid", type: "int", description: "The ID of the player" },
      { name: "text", type: "String", description: "The message text" },
    ],
    returnType: "int",
  },
  {
    name: "OnPlayerEnterVehicle",
    description: "Called when a player enters a vehicle",
    parameters: [
      { name: "playerid", type: "int", description: "The ID of the player" },
      { name: "vehicleid", type: "int", description: "The ID of the vehicle" },
      {
        name: "ispassenger",
        type: "bool",
        description: "Whether the player is a passenger",
      },
    ],
    returnType: "int",
  },
  {
    name: "OnPlayerExitVehicle",
    description: "Called when a player exits a vehicle",
    parameters: [
      { name: "playerid", type: "int", description: "The ID of the player" },
      { name: "vehicleid", type: "int", description: "The ID of the vehicle" },
    ],
    returnType: "int",
  },
];

/**
 * Get a callback by name
 */
export function getSampCallback(name: string): SampCallback | undefined {
  return SAMP_CALLBACKS.find((cb) => cb.name === name);
}

/**
 * Get all callback names
 */
export function getSampCallbackNames(): string[] {
  return SAMP_CALLBACKS.map((cb) => cb.name);
}
