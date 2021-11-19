import React from "react"
import LuaType from "./LuaType"
import Markdown from "./Markdown"
import styles from "./styles.module.css"

export default function LuaProp({
  luaClassName,
  name,
  desc,
  lua_type: luaType,
}) {
  return (
    <>
      <div className={styles.memberString}>
        <code>
          {luaClassName}.{name}:{" "}
        </code>{" "}
        <LuaType code={luaType} />
      </div>
      <Markdown content={desc} />
    </>
  )
}
