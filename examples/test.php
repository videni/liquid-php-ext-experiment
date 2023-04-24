<?php

class T {
    public function hello()
    {
        return "hello";
    }
}

$t = new T();

$tpl = Template::new("Liquid! {{name}} {{age}} {{t.hello()}}");

$vars = [
    'name' => 'David',
    'age' => 18,
];

$vars = new stdClass();
$vars->name = 'David';
$vars->age = 18;
$vars->t = $t;

var_dump($tpl->render($vars));
