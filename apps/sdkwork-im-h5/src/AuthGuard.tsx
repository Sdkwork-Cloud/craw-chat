import React, { useEffect, useState } from 'react';
import { Navigate, useLocation, useNavigate } from 'react-router';
import { AuthService } from '@sdkwork/im-h5-user';

export const AuthGuard: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [checking, setChecking] = useState(true);
  const location = useLocation();
  const navigate = useNavigate();

  useEffect(() => {
    // Just a small delay to simulate auth checking on load if needed
    setChecking(false);
  }, [location.pathname]);

  if (checking) return null;

  const currentUser = AuthService.getCurrentUser();

  if (!currentUser && location.pathname !== '/login') {
    return <Navigate to="/login" replace />;
  }

  return <>{children}</>;
};
