import React from "react"
import InlineDescription from "./InlineDescription"
import LuaType from "./LuaType"
import Markdown from "./Markdown"
import styles from "./styles.module.css"

const TypeAlias = ({ name, luaType, baseUrl, luaClassNames, robloxTypes }) => (
  <>
    <code className={styles.purple}>type</code> <code>{name} = </code>{" "}
    <LuaType
      code={luaType}
      baseUrl={baseUrl}
      luaClassNames={luaClassNames}
      robloxTypes={robloxTypes}
    />
  </>
)

const Param = ({ name, luaType, baseUrl, luaClassNames, robloxTypes }) => (
  <>
    <code>{name}:&nbsp;</code>
    <LuaType
      code={luaType}
      baseUrl={baseUrl}
      luaClassNames={luaClassNames}
      robloxTypes={robloxTypes}
    />
  </>
)

const Interface = ({ name, fields, baseUrl, luaClassNames, robloxTypes }) => (
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
            luaClassNames={luaClassNames}
            robloxTypes={robloxTypes}
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
  luaClassNames,
  robloxTypes,
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
            luaClassNames={luaClassNames}
            robloxTypes={robloxTypes}
          />
        ) : (
          <Interface
            name={name}
            fields={fields}
            baseUrl={baseUrl}
            luaClassNames={luaClassNames}
            robloxTypes={robloxTypes}
          />
        )}
      </div>
      <Markdown content={desc} />
    </>
  )
}
