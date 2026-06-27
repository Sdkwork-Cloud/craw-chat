export type ApiSchemaField = {
  name: string;
  type: string;
  description: string;
  required?: boolean | "conditional";
  summary?: string;
  children?: ApiSchemaField[];
};

export type ApiSchemaDefinition = {
  title?: string;
  fields: ApiSchemaField[];
};

export type ApiSchemaDefinitionMap = Record<string, ApiSchemaDefinition>;

type FieldOptions = Omit<ApiSchemaField, "name" | "type" | "description">;

export function field(
  name: string,
  type: string,
  description: string,
  options: FieldOptions = {},
): ApiSchemaField {
  return {
    name,
    type,
    description,
    ...options,
  };
}

export function objectField(
  name: string,
  description: string,
  children: ApiSchemaField[],
  options: Omit<FieldOptions, "children"> = {},
): ApiSchemaField {
  return field(name, "object", description, { ...options, children });
}

export function arrayField(
  name: string,
  itemType: string,
  description: string,
  children?: ApiSchemaField[],
  options: Omit<FieldOptions, "children"> = {},
): ApiSchemaField {
  return field(name, `${itemType}[]`, description, { ...options, children });
}
