# leptos-tutorial
- https://github.com/implrust/leptos-tutorial

# environment setup commands
- cargo install trunk
- rustup target add wasm32-unknown-unknown
- cargo install leptosfmt

# vscode plugins
- leptos-fmt
- leptos-vscode

# project setup
- cargo init leptos-tutorial
- cargo add leptos --features=csr
- cargo add console_error_panic_hook

# project run commands
- trunk serve open
- trunk serve --port 3000 --open

# leptos csr starter template
- https://github.com/leptos-rs/start-trunk

# cargo commands
- cargo --list

# content
- 3.1 A Basic Component ✅
    - signals 
    - ReadSignal, WriteSignal, RWSignal, 
    - .get(), .set(), .with(), .update(), .get_untracked()
<p align="center">
  <img src="./screenshots/ch3/3.1.basic-component.png" width="400" height="250" alt="Alt text">
</p>    

- 3.2 Dynamic Attributes ✅
    - trunk.toml
    - dynamic attributes, styles
    - set css variables for stylesheet
    - derived signals
    - injecting raw html
<p align="center">
  <img src="./screenshots/ch3/3.2.dynamic-attribute.png" width="400" height="300" alt="Alt text">
</p>     
<p align="center">
  <img src="./screenshots/ch3/3.2a.dynamic-attrs.png" width="400" alt="Alt text">
</p> 
<p align="center">
  <img src="./screenshots/ch3/3.2b.dynamic-attrs.png" width="400" alt="Alt text">
</p> 
<p align="center">
  <img src="./screenshots/ch3/3.2c.inject-rawhtml.png" width="480" alt="Alt text">
</p> 


- 3.3 Components and Props
    - components
    - props: optional, default,
    - passing value to components 
        - via props (signals, functions)
    - into props, optional generic props
    - documenting components
    - spread attr ref: https://github.com/leptos-rs/leptos/blob/main/examples/spread/src/lib.rs
<p align="center">
  <img src="./screenshots/ch3/3.3.components-props.png" width="450" height="450" alt="Alt text">
</p>    


- 3.4 Iteration
<h5>3.4.1 a Static Views - Static Content</h5>
<p align="center">
  <img src="./screenshots/ch3/3.4.1a.static-view.png" width="540" alt="Alt text">
</p>    
<h5>3.4.1 b Static Views - Dynamic Content</h5>
<p align="center">
  <img src="./screenshots/ch3/3.4.1b.static-view-dyn-content.png" width="540" alt="Alt text">
</p>    
<h5>3.4.2 Static List (Dynamic Content) - Dynamic List (Dynamic Content)</h5>
<p align="center">
  <img src="./screenshots/ch3/3.4.2.dyn-list-dyn-content.png" width="540" alt="Alt text">
</p>    
<h5>3.4.3 Accessing an index while iterating with ForEnumerate</h5>
<p align="center">
  <img src="./screenshots/ch3/3.4.3.for-enumerate.png" width="540" alt="Alt text">
</p>    