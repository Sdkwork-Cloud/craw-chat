const { Project, SyntaxKind } = require('ts-morph');

const project = new Project();
project.addSourceFilesAtPaths("packages/sdkwork-im-h5-notary/src/pages/NotaryDetail.tsx");

const chineseRegex = /[\u4e00-\u9fa5]/;

project.getSourceFiles().forEach(sourceFile => {
  let hasChanges = false;
  
  // Need to make sure 'useTranslation' is imported
  const imports = sourceFile.getImportDeclarations();
  const hasUseTranslation = imports.some(i => i.getModuleSpecifierValue() === 'react-i18next');
  
  if (hasUseTranslation) {
    // Find functional component body and make sure `const { t } = useTranslation();` exists.
    // For simplicity, we just look for JsxText first
    sourceFile.getDescendantsOfKind(SyntaxKind.JsxText).forEach(jsxText => {
      const text = jsxText.getText();
      if (chineseRegex.test(text)) {
        // e.g. "正在加载公证详情..."
        const trimmed = text.trim();
        if (trimmed.length > 0) {
          // Replace JSXText with JSXExpression
          const key = 'notary.detail.' + Math.random().toString(36).substr(2, 5); // temporary auto-gen key
          // Wait, ts-morph `replaceWithText` is tricky with JSXText since it replaces the exact AST node text.
          // Better to use `replaceWithText(`{t('${key}', '${trimmed}')}`)`
          jsxText.replaceWithText(`{t('${key}', '${trimmed}')}`);
          hasChanges = true;
        }
      }
    });
  }

  if (hasChanges) {
    sourceFile.saveSync();
    console.log(`Saved ${sourceFile.getFilePath()}`);
  }
});
