export interface IdeColumn {
  name: string;
  type: string;
  itemsType?: string;
  required: boolean;
  default?: any;
  description?: string;
  variableLength?: boolean;
  dependsOn?: string;
  format?: string;
  games?: string[];
}

export interface IdeSectionType {
  id?: number;
  name?: string;
  columns?: IdeColumn[];
  games?: string[];
}

export interface IdeSection {
  supportedGames: string[];
  primaryKeys: string[];
  description: string;
  columns: IdeColumn[];
  commonPrefix?: IdeColumn[];
  types?: Record<string, IdeSectionType>;
  discriminator?: boolean;
  parseHints?: Record<string, any>;
  variants?: Record<string, IdeVariant>;
}

export interface IdeVariant {
  insertAfter: string;
  description: string;
  extraColumns: IdeColumn[];
}

export interface IdeSchema {
  sections: Record<string, IdeSection>;
}

export interface IdeRow {
  data: Record<string, any>;
  extraFields?: string[];
  raw?: string;
}

export interface IdeSectionData {
  rows: IdeRow[];
  errors: string[];
}

export interface IdeDocument {
  filePath: string;
  sections: Record<string, IdeSectionData>;
}

export interface IdeEditOperation {
  section: string;
  operation: "add" | "delete" | "update";
  rowIndex?: number;
  data?: Record<string, any>;
}

export interface IdeValidationError {
  section: string;
  rowIndex: number;
  column?: string;
  message: string;
}

export interface IdeViewMode {
  mode: "structured" | "raw";
  expandedSections?: Set<string>;
}
