import DefaultTheme from "vitepress/theme";
import ApiSchemaTable from "./components/ApiSchemaTable.vue";
import "./custom.css";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component("ApiSchemaTable", ApiSchemaTable);
  },
};
