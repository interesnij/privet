function get_document_opacity_0() {
  document.body.style.overflow = "hidden";
  document.body.style.marginRight = "4px";
  overlay = document.body.querySelector(".body_overlay");
  overlay.style.visibility = "unset";
  overlay.style.opacity = "1";
};
function get_document_opacity_1() {
  document.body.style.overflow = "scroll";
  document.body.style.marginRight = "0";
  overlay = document.body.querySelector(".body_overlay");
  overlay.style.visibility = "hidden";
  overlay.style.opacity = "0";
};

function close_fullscreen() {
  container = document.body.querySelector("#fullscreens_container");
  if (!container.innerHTML) {
    get_document_opacity_1();
    return
  };
  container = document.body.querySelector("#fullscreens_container");
  _window = container.querySelector(".card_fullscreen");
  _window.remove();

  if (!container.innerHTML) {
    get_document_opacity_1();
  } else {
    prev_window = container.querySelector(".card_fullscreen");
    prev_window.querySelector(".this_fullscreen_hide").style.display = "unset";
    try {prev_window.querySelector(".prev_item").style.display = "unset"} catch {null};
    try {prev_window.querySelector(".next_item").style.display = "unset"} catch {null}
  };
};

function create_fullscreen(url, type_class) {
  container = document.body.querySelector("#fullscreens_container");

  if (container.innerHTML) {
    prev_window = container.querySelector(".card_fullscreen");
    prev_window.querySelector(".this_fullscreen_hide").style.display = "none";
    try {prev_window.querySelector(".prev_item").style.display = "none"} catch {null};
    try {prev_window.querySelector(".next_item").style.display = "none"} catch {null}
  };

  try {
    count_items = container.querySelectorAll(".card_fullscreen").length + 1
  } catch {count_items = 0};

  $parent_div = document.createElement("div");
  $parent_div.classList.add("card_fullscreen", "mb-30", "border", type_class);
  $parent_div.style.zIndex = 100 + count_items;
  $parent_div.style.opacity = "0";

  if (document.body.querySelector(".desctop_nav")) {
    hide_svg = '<svg class="svg_default" style="position:fixed;" width="30" height="30" fill="currentColor" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>'
  } else { hide_svg = "" };
  $hide_span = document.createElement("span");
  $hide_span.classList.add("this_fullscreen_hide");
  $loader = document.createElement("div");

  $loader.setAttribute("id", "fullscreen_loader");
  $hide_span.innerHTML = hide_svg;
  $parent_div.append($hide_span);
  $parent_div.append($loader);
  container.prepend($parent_div);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link.open('GET', url, true);
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          elem = link.responseText;

          $loader.innerHTML = elem;
          height = $loader.scrollHeight*1 + 30;
          if (height < 500 && !$loader.querySelector(".data_display")) {
            $parent_div.style.height = height + "px";
            $loader.style.overflowY = "unset";

            _height = (window.innerHeight - height - 50) / 2;
            $parent_div.style.top = _height + "px";
            prev_next_height = _height*1 + 50 + "px";
            try {$loader.querySelector(".prev_item").style.top = "-" + prev_next_height}catch {null};
            try {$loader.querySelector(".next_item").style.top = "-" + prev_next_height}catch {null}
          } else {
            $parent_div.style.height = "100%";
            $parent_div.style.top = "15px";
            $loader.style.overflowY = "auto";
          };
          $parent_div.style.opacity = "1";
          if ($loader.querySelector(".data_display")) {
            $loader.style.overflowY = "unset";
          }

          get_document_opacity_0();

          $loader.onscroll = function() {
            if ($loader.querySelector(".next_page_list")) {
              box = $loader.querySelector('.next_page_list');
              if (box && box.classList.contains("next_page_list")) {
                  inViewport = elementInViewport(box);
                  if (inViewport) {
                      box.remove();
                      var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
                      link_3.open('GET', location.protocol + "//" + location.host + box.getAttribute("data-link"), true);
                      link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

                      link_3.onreadystatechange = function() {
                          if (this.readyState == 4 && this.status == 200) {
                              var elem = document.createElement('span');
                              elem.innerHTML = link_3.responseText;
                              $loader.querySelector(".is_block_paginate").insertAdjacentHTML('beforeend', elem.querySelector(".is_block_paginate").innerHTML);
                            }
                      }
                      link_3.send();
                  }
              };
            };
          };
      }
  };
  link.send();
};

function change_this_fullscreen(_this, type_class) {
  _this.parentElement.classList.contains("col") ? $loader = _this.parentElement.parentElement.parentElement.parentElement : $loader = _this.parentElement.parentElement;
  $loader.innerHTML = "";
  $parent_div.style.opacity = "0";
  $parent_div.style.height = "35px";
  url = _this.getAttribute("href");

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link.open('GET', url, true);
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          elem = link.responseText;
          $loader.innerHTML = elem;
          height = $loader.scrollHeight*1 + 30;
          $parent_div = $loader.parentElement
          if (height < 500 && !$loader.querySelector(".data_display")){
            $parent_div.style.height = height + "px";
            _height = (window.innerHeight - height - 50) / 2;
            $parent_div.style.top = _height + "px";
            prev_next_height = _height*1 + 50 + "px";
            $loader.style.overflowY = "unset";
            try {$loader.querySelector(".prev_item").style.top = "-" + prev_next_height}catch {null};
            try {$loader.querySelector(".next_item").style.top = "-" + prev_next_height}catch {null}
          } else {
            $parent_div.style.height = "100%";
            $parent_div.style.top = "15px";
            $loader.style.overflowY = "auto";
          };
          $parent_div.style.opacity = "1";
          $parent_div.style.opacity = "1";
          if ($loader.querySelector(".data_display")) {
            $loader.style.overflowY = "unset";
          };

          get_document_opacity_0();

          $loader.onscroll = function() {
            if ($loader.querySelector(".next_page_list")) {
              box = $loader.querySelector('.next_page_list');
              if (box && box.classList.contains("next_page_list")) {
                  inViewport = elementInViewport(box);
                  if (inViewport) {
                      box.remove();
                      var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
                      link_3.open('GET', location.protocol + "//" + location.host + box.getAttribute("data-link"), true);
                      link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

                      link_3.onreadystatechange = function() {
                          if (this.readyState == 4 && this.status == 200) {
                              var elem = document.createElement('span');
                              elem.innerHTML = link_3.responseText;
                              $loader.querySelector(".is_block_paginate").insertAdjacentHTML('beforeend', elem.querySelector(".is_block_paginate").innerHTML);
                            }
                      }
                      link_3.send();
                  }
              };
            };
          }
      }
  };
  link.send();
};

function addStyleSheets(href) {
    $head = document.head, $link = document.createElement('link');
    $link.rel = 'stylesheet';
    $link.classList.add("color");
    $link.href = href;
    $head.appendChild($link)
};

function init_wow(){
    var wow = new WOW({
      boxClass: 'wow',
      animateClass: 'animated',
      offset: 90,
      mobile: false,
      live: true
  });
  wow.init();
};

class ToastManager {
    constructor() {
        this.id = 0;
        this.toasts = [];
        this.icons = {
            'SUCCESS': "",
            'ERROR': '',
            'INFO': '',
            'WARNING': '',
        };
        var body = document.querySelector('#ajax');
        this.toastsContainer = document.createElement('div');
        this.toastsContainer.classList.add('toasts', 'border-0');
        body.appendChild(this.toastsContainer)
    }
    showSuccess(message) {
        return this._showToast(message, 'SUCCESS')
    }
    showError(message) {
        return this._showToast(message, 'ERROR')
    }
    showInfo(message) {
        return this._showToast(message, 'INFO')
    }
    showWarning(message) {
        return this._showToast(message, 'WARNING')
    }
    _showToast(message, toastType) {
        var newId = this.id + 1;
        var newToast = document.createElement('div');
        newToast.style.display = 'inline-block';
        newToast.classList.add(toastType.toLowerCase());
        newToast.classList.add('toast');
        newToast.innerHTML = `<progress max="100"value="0"></progress><h3>${message}</h3>`;
        var newToastObject = {
            id: newId,
            message,
            type: toastType,
            timeout: 4000,
            progressElement: newToast.querySelector('progress'),
            counter: 0,
            timer: setInterval(() => {
                newToastObject.counter += 1000 / newToastObject.timeout;
                newToastObject.progressElement.value = newToastObject.counter.toString();
                if (newToastObject.counter >= 100) {
                    newToast.style.display = 'none';
                    clearInterval(newToastObject.timer);
                    this.toasts = this.toasts.filter((toast) => {
                        return toast.id === newToastObject.id
                    })
                }
            }, 10)
        }
        newToast.addEventListener('click', () => {
            newToast.style.display = 'none';
            clearInterval(newToastObject.timer);
            this.toasts = this.toasts.filter((toast) => {
                return toast.id === newToastObject.id
            })
        });
        this.toasts.push(newToastObject);
        this.toastsContainer.appendChild(newToast);
        return this.id++
    }
}

function toast_success(text) {
    var toasts = new ToastManager();
    toasts.showSuccess(text)
}

function toast_error(text) {
    var toasts = new ToastManager();
    toasts.showError(text)
}

function toast_info(text) {
    var toasts = new ToastManager();
    toasts.showInfo(text)
}

function toast_warning(text) {
    var toasts = new ToastManager();
    toasts.showWarning(text)
}

function service_tab_action(is, tab_class){
  if (!is.classList.contains("active")){
  nav = is.parentElement.parentElement.parentElement;
  nav_items = document.body.querySelectorAll(".yy");
  for (var i = 0; i < nav_items.length; i++){nav_items[i].classList.remove("active"); nav_items[i].classList.add("pointer")};
  is.classList.add("active"); is.classList.remove("pointer");
  tabs = document.body.querySelector(".tab-content");
  tabs_items = tabs.querySelectorAll(".tab-pane");
  for (var i = 0; i < tabs_items.length; i++){tabs_items[i].classList.remove("active")};
  cur = tabs.querySelector(tab_class);
  cur.classList.add("active")
}};

function on(elSelector, eventName, selector, fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while (el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});};

function get_custom_design() {
  color = "white";
  params = window.location.search.replace( '?', '').split('&');
    if (params[0] && params[0].split("=")[0] == "f") {
      color = params[0].split("=")[1] }
    else if (params[1] && params[1].split("=")[0] == "f") {
      color = params[1].split("=")[1];
    };

    if (color == "white") {
        addStyleSheets("/static/styles/color/white.css")
    } else if (color == "dark") {
        addStyleSheets("/static/styles/color/dark.css")
    } else if (color == "yellow") {
        addStyleSheets("/static/styles/color/yellow.css")
    } else if (color == "white_kletka") {
        addStyleSheets("/static/styles/color/white_kletka.css")
    } else if (color == "dark_wood") {
        addStyleSheets("/static/styles/color/dark_wood.css")
    };
    btn = document.body.querySelector(".anon_color_change");
    btn.setAttribute("data-color", color)
};
get_custom_design();

on('body', 'click', '.ajax', function(event) {
  //this.classList.remove("ajax");
  event.preventDefault();
  var url = this.getAttribute('href');
  if (url != window.location.pathname){
    ajax_get_reload(url);
  } else {toast_info("Вы уже на этой странице")}
});

init_wow();

on('#ajax', 'click', '.s_1', function() {
  service_tab_action(this, ".tab_1")
});
on('#ajax', 'click', '.s_2', function() {
  service_tab_action(this, ".tab_2")
});
on('#ajax', 'click', '.s_3', function() {
  service_tab_action(this, ".tab_3")
});
on('#ajax', 'click', '.s_4', function() {
  service_tab_action(this, ".tab_4")
});
on('#ajax', 'click', '.s_5', function() {
  service_tab_action(this, ".tab_5")
});
on('#ajax', 'click', '.s_6', function() {
  service_tab_action(this, ".tab_6")
});
on('#ajax', 'click', '.s_7', function() {
  service_tab_action(this, ".tab_7")
});

on('body', 'click', '.anon_color_change', function() {
  window.history.replaceState(null, null, window.location.pathname);
  color = this.getAttribute("data-color");
  if (color == "white") {
    addStyleSheets("/static/styles/color/dark.css");
    this.setAttribute("data-color", "dark");
    new_color = "dark"
  } else if (color == "dark") {
    addStyleSheets("/static/styles/color/yellow.css");
    this.setAttribute("data-color", "yellow");
    new_color = "yellow"
  } else if (color == "yellow") {
    addStyleSheets("/static/styles/color/white_kletka.css");
    this.setAttribute("data-color", "white_kletka");
    new_color = "white_kletka"
  } else if (color == "white_kletka") {
    addStyleSheets("/static/styles/color/dark_wood.css");
    this.setAttribute("data-color", "dark_wood");
    new_color = "dark_wood"
  } else if (color == "dark_wood") {
    addStyleSheets("/static/styles/color/white.css");
    this.setAttribute("data-color", "white");
    new_color = "white"
  };
  window.history.replaceState(null, null, window.location.pathname + "?content=''&f=" + new_color);
});
on('body', 'click', '.this_fullscreen_hide', function() {
  close_fullscreen()
});
on('body', 'click', '.body_overlay', function() {
  close_fullscreen()
});

on('#ajax', 'click', '.get_object_info', function() {
  create_fullscreen("/load_item/?_type=" + this.getAttribute("data-type") + "&_pk=" + this.getAttribute("data-pk"), "worker_fullscreen");
});
