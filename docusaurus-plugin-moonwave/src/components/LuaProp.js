import React from "react"
import LuaType from "./LuaType"
import Markdown from "./Markdown"
import styles from "./styles.module.css"

export default function LuaProp({
  luaClassName,
  typeLinks,
  name,
  desc,
  lua_type: luaType,
  baseUrl,
}) {
  return (
    <>
      <div className={styles.memberString}>
        <code>
          {luaClassName}.{name}:{" "}
        </code>{" "}
        <LuaType code={luaType} baseUrl={baseUrl} typeLinks={typeLinks} />
      </div>
      <Markdown content={desc} />
    </>
  )
}
