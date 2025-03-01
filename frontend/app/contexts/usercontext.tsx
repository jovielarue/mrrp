"use client";
import {
  createContext,
  Dispatch,
  ReactNode,
  SetStateAction,
  useState,
} from "react";

interface UserContextType {
  username: string;
  setContextUsername: Dispatch<SetStateAction<string>>;
  authToken: string;
  setAuthToken: Dispatch<SetStateAction<string>>;
}
export const UserContext = createContext({} as UserContextType);

export default function UserContextProvider(props: { children: ReactNode }) {
  const [username, setContextUsername] = useState("");
  const [authToken, setAuthToken] = useState("");

  return (
    <UserContext.Provider
      value={{
        username,
        setContextUsername,
        authToken,
        setAuthToken,
      }}
    >
      {props.children}
    </UserContext.Provider>
  );
}
