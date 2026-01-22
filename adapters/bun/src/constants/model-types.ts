export const ModelType = {
  OBJECT: "object",
  SKIN: "skin",
  CAR: "car",
} as const;

export type ModelType = (typeof ModelType)[keyof typeof ModelType];
