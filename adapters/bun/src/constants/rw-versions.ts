export const RwVersion = {
  SA: 221_187,
  VC: 212_995,
  III: 208_898,
} as const;

export type RwVersion = (typeof RwVersion)[keyof typeof RwVersion];
