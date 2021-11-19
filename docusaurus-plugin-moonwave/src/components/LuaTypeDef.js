import React from "react"
import InlineDescription from "./InlineDescription"
import LuaType from "./LuaType"
import Markdown from "./Markdown"
import styles from "./styles.module.css"

const TypeAlias = ({ name, luaType, baseUrl, typeLinks }) => (
  <>
    <code className={styles.purple}>type</code> <code>{name} = </code>{" "}
    <LuaType code={luaType} baseUrl={baseUrl} typeLinks={typeLinks} />
  </>
)

const Param = ({ name, luaType, baseUrl, typeLinks }) => (
  <>
    <code>{name}:&nbsp;</code>
    <LuaType code={luaType} baseUrl={baseUrl} typeLinks={typeLinks} />
  </>
)

const Interface = ({ name, fields, baseUrl, typeLinks }) => (
  <>
    <code className={styles.purple}>interface</code>{" "}
    <code>
      {name} {"{"}
    </code>
    <div className={styles.inset}>
      {fields.map(({ name, lua_type: luaType, desc }) => (
        <div key={name}>
          <Param
            name={name}
            luaType={luaType}
            baseUrl={baseUrl}
            typeLinks={typeLinks}
          />
          {desc && <InlineDescription content={desc} />}
        </div>
      ))}
    </div>
    <code>{"}"}</code>
  </>
)

export default function LuaTypeDef({
  luaClassName,
  typeLinks,
  name,
  desc,
  lua_type: luaType,
  fields,
  baseUrl,
}) {
  return (
    <>
      <div className={styles.memberString}>
        {luaType ? (
          <TypeAlias
            name={name}
            luaType={luaType}
            baseUrl={baseUrl}
            typeLinks={typeLinks}
          />
        ) : (
          <Interface
            name={name}
            fields={fields}
            baseUrl={baseUrl}
            typeLinks={typeLinks}
          />
        )}
      </div>
      <Markdown content={desc} />
    </>
  )
}
