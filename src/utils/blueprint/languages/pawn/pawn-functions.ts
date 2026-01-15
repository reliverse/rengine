/**
 * SAMP native function library definitions
 */

export interface SampNativeFunction {
  name: string;
  description: string;
  parameters: SampFunctionParameter[];
  returnType: string;
  category: string;
}

export interface SampFunctionParameter {
  name: string;
  type: string;
  description?: string;
  optional?: boolean;
}

export const SAMP_NATIVES: SampNativeFunction[] = [
  // Object Functions
  {
    name: "CreateDynamicObject",
    description: "Creates a dynamic object in the world",
    parameters: [
      {
        name: "modelid",
        type: "int",
        description: "The model ID of the object",
      },
      { name: "x", type: "Float", description: "X coordinate" },
      { name: "y", type: "Float", description: "Y coordinate" },
      { name: "z", type: "Float", description: "Z coordinate" },
      { name: "rx", type: "Float", description: "Rotation X", optional: true },
      { name: "ry", type: "Float", description: "Rotation Y", optional: true },
      { name: "rz", type: "Float", description: "Rotation Z", optional: true },
      {
        name: "worldid",
        type: "int",
        description: "Virtual world ID",
        optional: true,
      },
      {
        name: "interiorid",
        type: "int",
        description: "Interior ID",
        optional: true,
      },
      {
        name: "playerid",
        type: "int",
        description: "Player ID",
        optional: true,
      },
      {
        name: "streamdistance",
        type: "Float",
        description: "Stream distance",
        optional: true,
      },
      {
        name: "drawdistance",
        type: "Float",
        description: "Draw distance",
        optional: true,
      },
    ],
    returnType: "int",
    category: "Objects",
  },
  {
    name: "DestroyDynamicObject",
    description: "Destroys a dynamic object",
    parameters: [
      {
        name: "objectid",
        type: "int",
        description: "The object ID to destroy",
      },
    ],
    returnType: "int",
    category: "Objects",
  },
  // Player Functions
  {
    name: "SetPlayerPos",
    description: "Sets a player's position",
    parameters: [
      { name: "playerid", type: "int", description: "The player ID" },
      { name: "x", type: "Float", description: "X coordinate" },
      { name: "y", type: "Float", description: "Y coordinate" },
      { name: "z", type: "Float", description: "Z coordinate" },
    ],
    returnType: "int",
    category: "Players",
  },
  {
    name: "GetPlayerPos",
    description: "Gets a player's position",
    parameters: [
      { name: "playerid", type: "int", description: "The player ID" },
      { name: "x", type: "Float", description: "X coordinate (by reference)" },
      { name: "y", type: "Float", description: "Y coordinate (by reference)" },
      { name: "z", type: "Float", description: "Z coordinate (by reference)" },
    ],
    returnType: "int",
    category: "Players",
  },
  {
    name: "SetPlayerHealth",
    description: "Sets a player's health",
    parameters: [
      { name: "playerid", type: "int", description: "The player ID" },
      { name: "health", type: "Float", description: "Health value (0-100)" },
    ],
    returnType: "int",
    category: "Players",
  },
  {
    name: "GetPlayerHealth",
    description: "Gets a player's health",
    parameters: [
      { name: "playerid", type: "int", description: "The player ID" },
      {
        name: "health",
        type: "Float",
        description: "Health value (by reference)",
      },
    ],
    returnType: "int",
    category: "Players",
  },
  {
    name: "GivePlayerMoney",
    description: "Gives money to a player",
    parameters: [
      { name: "playerid", type: "int", description: "The player ID" },
      { name: "money", type: "int", description: "Amount of money to give" },
    ],
    returnType: "int",
    category: "Players",
  },
  {
    name: "GetPlayerMoney",
    description: "Gets a player's money",
    parameters: [
      { name: "playerid", type: "int", description: "The player ID" },
    ],
    returnType: "int",
    category: "Players",
  },
  // Vehicle Functions
  {
    name: "CreateVehicle",
    description: "Creates a vehicle",
    parameters: [
      { name: "vehicletype", type: "int", description: "Vehicle model ID" },
      { name: "x", type: "Float", description: "X coordinate" },
      { name: "y", type: "Float", description: "Y coordinate" },
      { name: "z", type: "Float", description: "Z coordinate" },
      {
        name: "rotation",
        type: "Float",
        description: "Rotation angle",
        optional: true,
      },
      {
        name: "color1",
        type: "int",
        description: "Primary color",
        optional: true,
      },
      {
        name: "color2",
        type: "int",
        description: "Secondary color",
        optional: true,
      },
    ],
    returnType: "int",
    category: "Vehicles",
  },
  {
    name: "DestroyVehicle",
    description: "Destroys a vehicle",
    parameters: [
      { name: "vehicleid", type: "int", description: "The vehicle ID" },
    ],
    returnType: "int",
    category: "Vehicles",
  },
  // Chat Functions
  {
    name: "SendClientMessage",
    description: "Sends a message to a player",
    parameters: [
      { name: "playerid", type: "int", description: "The player ID" },
      { name: "color", type: "int", description: "Message color" },
      { name: "message", type: "String", description: "The message text" },
    ],
    returnType: "int",
    category: "Chat",
  },
  {
    name: "SendClientMessageToAll",
    description: "Sends a message to all players",
    parameters: [
      { name: "color", type: "int", description: "Message color" },
      { name: "message", type: "String", description: "The message text" },
    ],
    returnType: "int",
    category: "Chat",
  },
  // Utility Functions
  {
    name: "printf",
    description: "Prints formatted text to server console",
    parameters: [
      { name: "format", type: "String", description: "Format string" },
    ],
    returnType: "int",
    category: "Utility",
  },
  {
    name: "format",
    description: "Formats a string",
    parameters: [
      {
        name: "output",
        type: "String",
        description: "Output string (by reference)",
      },
      { name: "len", type: "int", description: "Maximum length" },
      { name: "format", type: "String", description: "Format string" },
    ],
    returnType: "int",
    category: "Utility",
  },
];

/**
 * Get a native function by name
 */
export function getSampNative(name: string): SampNativeFunction | undefined {
  return SAMP_NATIVES.find((fn) => fn.name === name);
}

/**
 * Get all native functions
 */
export function getAllSampNatives(): SampNativeFunction[] {
  return SAMP_NATIVES;
}

/**
 * Get native functions by category
 */
export function getSampNativesByCategory(
  category: string
): SampNativeFunction[] {
  return SAMP_NATIVES.filter((fn) => fn.category === category);
}

/**
 * Get all categories
 */
export function getSampNativeCategories(): string[] {
  return Array.from(new Set(SAMP_NATIVES.map((fn) => fn.category)));
}
