<template>
  <div class="navbar bg-base-100">
    <div class="flex-1">
      <RouterLink class="btn btn-ghost normal-case text-xl"
                  to="/">Todo</RouterLink>
    </div>
    <div class="flex-none">
      <ul class="menu menu-horizontal px-1">
        <li><RouterLink to="/about">About</RouterLink></li>
      </ul>
    </div>
  </div>
</template>