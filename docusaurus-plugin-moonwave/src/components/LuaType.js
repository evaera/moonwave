import React from "react"
import styles from "./styles.module.css"

export default function LuaType({ children }) {
  return <code className={styles.blue}>{children}</code>
}
