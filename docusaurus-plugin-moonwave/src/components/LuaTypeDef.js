import React from "react"
import InlineDescription from "./InlineDescription"
import LuaType from "./LuaType"
import Markdown from "./Markdown"
import styles from "./styles.module.css"

const TypeAlias = ({ name, luaType }) => (
  <>
    <code className={styles.purple}>type</code> <code>{name} = </code>{" "}
    <LuaType code={luaType} />
  </>
)

const Param = ({ name, luaType }) => (
  <>
    <code>{name}:&nbsp;</code>
    <LuaType code={luaType} />
  </>
)

const Interface = ({ name, fields }) => (
  <>
    <code className={styles.purple}>interface</code>{" "}
    <code>
      {name} {"{"}
    </code>
    <div className={styles.inset}>
      {fields.map(({ name, lua_type: luaType, desc }) => (
        <div key={name}>
          <Param name={name} luaType={luaType} />
          {desc && <InlineDescription content={desc} />}
        </div>
      ))}
    </div>
    <code>{"}"}</code>
  </>
)

export default function LuaTypeDef({
  luaClassName,
  name,
  desc,
  lua_type: luaType,
  fields,
}) {
  return (
    <>
      <div className={styles.memberString}>
        {luaType ? (
          <TypeAlias name={name} luaType={luaType} />
        ) : (
          <Interface name={name} fields={fields} />
        )}
      </div>
      <Markdown content={desc} />
    </>
  )
}
