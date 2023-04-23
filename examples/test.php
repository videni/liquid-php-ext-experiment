<?php

var_dump(hello_world("David"));

class T {
    public function hello()
    {
        return "hello";
    }
}

$t = new T();

var_dump(take_obj($t)->hello());

$tpl = Template::new("Liquid! {{num | minus: 2}}");

$vars = [
    'name' => 'David',
    'age' => 18,
];

$vars = new stdClass();
$vars->name = 'David';
$vars->age = 18;
$vars->t = $t;

$tpl->render($vars);
