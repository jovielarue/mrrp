"use client";
import { createContext, ReactNode } from "react";

interface UserContextType {
  handleSetAuthToken: (token: string) => void;
  handleGetAuthToken: () => string;
  handleSetUsername: (username: string) => void;
  handleGetUsername: () => string;
}
export const UserContext = createContext({} as UserContextType);

export default function UserContextProvider(props: { children: ReactNode }) {
  const handleSetAuthToken = (token: string) => {
    localStorage.setItem("jwt", token);
  };

  const handleGetAuthToken = () => {
    const token = localStorage.getItem("jwt");
    return token || "";
  };

  const handleSetUsername = (username: string) => {
    localStorage.setItem("username", username);
  };

  const handleGetUsername = () => {
    const username = localStorage.getItem("username");
    return username || "";
  };

  return (
    <UserContext.Provider
      value={{
        handleSetAuthToken,
        handleGetAuthToken,
        handleSetUsername,
        handleGetUsername,
      }}
    >
      {props.children}
    </UserContext.Provider>
  );
}
