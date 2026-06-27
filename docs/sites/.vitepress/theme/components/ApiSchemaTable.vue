<script setup lang="ts">
import { computed, ref } from "vue";
import {
  apiSchemas,
  type ApiSchemaDefinition,
  type ApiSchemaField,
} from "../api-schemas";

defineOptions({ name: "ApiSchemaTable" });

const props = withDefaults(
  defineProps<{
    schema?: string;
    fields?: ApiSchemaField[];
    level?: number;
  }>(),
  {
    level: 0,
  },
);

const definition = computed<ApiSchemaDefinition>(() => {
  if (props.fields?.length) {
    return { fields: props.fields };
  }
  if (!props.schema) {
    return { fields: [] };
  }

  const resolved = apiSchemas[props.schema];
  if (!resolved) {
    throw new Error(`Unknown API schema: ${props.schema}`);
  }

  return resolved;
});

const rootEl = ref<HTMLElement | null>(null);
const hasNestedFields = computed(() =>
  definition.value.fields.some((field) => (field.children?.length ?? 0) > 0),
);

function requiredLabel(required?: boolean | "conditional") {
  if (required === undefined) {
    return "-";
  }
  if (required === "conditional") {
    return "Conditional";
  }
  return required ? "Yes" : "No";
}

function nestedKind(field: ApiSchemaField) {
  return field.type.includes("[]") ? "array" : "object";
}

function nestedCount(field: ApiSchemaField) {
  return field.children?.length ?? 0;
}

function nestedSummary(field: ApiSchemaField) {
  if (field.summary) {
    return field.summary;
  }

  const count = nestedCount(field);
  const target = nestedKind(field) === "array" ? "array item fields" : "nested fields";
  return `View ${target} for ${field.name}${count ? ` (${count})` : ""}`;
}

function nestedCountLabel(field: ApiSchemaField) {
  const count = nestedCount(field);
  if (!count) {
    return "Expandable";
  }

  return `${count} ${count === 1 ? "field" : "fields"}`;
}

function setAllNestedDetails(open: boolean) {
  const root = rootEl.value;
  if (!root) {
    return;
  }

  for (const node of root.querySelectorAll("details.api-schema-details")) {
    const details = node as HTMLDetailsElement;
    details.open = open;
  }
}
</script>

<template>
  <div ref="rootEl" class="api-schema-table" :data-level="level">
    <div v-if="level === 0 && hasNestedFields" class="api-schema-toolbar">
      <button
        type="button"
        class="api-schema-toolbar-btn"
        @click="setAllNestedDetails(true)"
      >
        Expand all nested fields
      </button>
      <button
        type="button"
        class="api-schema-toolbar-btn is-secondary"
        @click="setAllNestedDetails(false)"
      >
        Collapse all
      </button>
    </div>
    <table>
      <thead>
        <tr>
          <th>Field</th>
          <th>Type</th>
          <th>Required</th>
          <th>Description</th>
        </tr>
      </thead>
      <tbody>
        <template v-for="field in definition.fields" :key="field.name">
          <tr>
            <td>
              <div class="api-schema-field">
                <code>{{ field.name }}</code>
                <span
                  v-if="field.children?.length"
                  class="api-schema-kind"
                  :data-kind="nestedKind(field)"
                >
                  {{ nestedKind(field) }}
                </span>
              </div>
            </td>
            <td><code>{{ field.type }}</code></td>
            <td>{{ requiredLabel(field.required) }}</td>
            <td>{{ field.description }}</td>
          </tr>
          <tr v-if="field.children?.length" class="api-schema-nested-row">
            <td colspan="4">
              <details class="api-schema-details">
                <summary>
                  <span class="api-schema-summary-heading">
                    <code>{{ field.name }}</code>
                    <span class="api-schema-kind" :data-kind="nestedKind(field)">
                      {{ nestedKind(field) }}
                    </span>
                    <span class="api-schema-count">{{ nestedCountLabel(field) }}</span>
                  </span>
                  <span class="api-schema-summary-copy">{{ nestedSummary(field) }}</span>
                </summary>
                <div class="api-schema-body">
                  <ApiSchemaTable :fields="field.children" :level="level + 1" />
                </div>
              </details>
            </td>
          </tr>
        </template>
      </tbody>
    </table>
  </div>
</template>
