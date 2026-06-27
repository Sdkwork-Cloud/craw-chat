import React from 'react';
import { I18nextProvider } from 'react-i18next';
import i18n from './i18n';
import { EnterpriseView as InternalEnterpriseView, EnterpriseViewProps } from './EnterpriseView';

export type { EnterpriseViewProps } from './EnterpriseView';
export * from './components/EnterpriseDetail';

export const EnterpriseView: React.FC<EnterpriseViewProps> = (props) => (
  <I18nextProvider i18n={i18n}>
    <InternalEnterpriseView {...props} />
  </I18nextProvider>
);
