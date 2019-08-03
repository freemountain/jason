 function(name, port = 90) {
    name: name,
    port: port,
    a: {
        b: port * 2
    },
    // Sidenote: the fields are hidden here, because they use ::,
    // use : to make them appear when the object is manifested.
    // Sidenote 2: You can provide default argument values.
    // For example function(name, port=8080) makes port optional.
  }