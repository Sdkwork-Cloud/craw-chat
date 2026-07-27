import i18next from 'i18next';
const t = i18next.t.bind(i18next);
export interface AuthUser {
  id: string;
  phone: string;
  token: string;
}

let currentUser: AuthUser | null = null;
try {
  const saved = localStorage.getItem("auth_user");
  if (saved) currentUser = JSON.parse(saved);
} catch (e) {}

export const AuthService = {
  async login(
    phone: string,
    password?: string,
    code?: string,
  ): Promise<AuthUser> {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        if (password === "123456" || code === "8888") {
          currentUser = {
            id: `u_${Date.now()}`,
            phone,
            token: `token_${Date.now()}`,
          };
          localStorage.setItem("auth_user", JSON.stringify(currentUser));
          resolve(currentUser);
        } else {
          reject(new Error(t("auth.error_invalid_pwd", "账号或密码错误（演示密码：123456，验证码：8888）")));
        }
      }, 500);
    });
  },

  async register(
    phone: string,
    code: string,
    password?: string,
  ): Promise<AuthUser> {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        if (code === "8888") {
          currentUser = {
            id: `u_${Date.now()}`,
            phone,
            token: `token_${Date.now()}`,
          };
          localStorage.setItem("auth_user", JSON.stringify(currentUser));
          resolve(currentUser);
        } else {
          reject(new Error(t("auth.error_invalid_code", "验证码错误（演示验证码：8888）")));
        }
      }, 500);
    });
  },

  async resetPassword(
    phone: string,
    code: string,
    newPassword: string,
  ): Promise<boolean> {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        if (code === "8888") {
          resolve(true);
        } else {
          reject(new Error(t("auth.error_invalid_code", "验证码错误（演示验证码：8888）")));
        }
      }, 500);
    });
  },

  async sendCode(phone: string): Promise<boolean> {
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve(true); // Always succeds in mock
      }, 300);
    });
  },

  async logout(): Promise<void> {
    return new Promise((resolve) => {
      setTimeout(() => {
        currentUser = null;
        localStorage.removeItem("auth_user");
        resolve();
      }, 300);
    });
  },

  getCurrentUser(): AuthUser | null {
    return currentUser;
  },
};
