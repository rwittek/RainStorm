#ifndef _CONTROL_VARIABLES_H_
#define _CONTROL_VARIABLES_H_
//===================================================================================
class CControlVariables
{
public:

	void Initialize( void );
	void Load( void );
	void Save( void );

#if COMPILE_ESP
	float esp_health;
	float esp_class;
	float esp_dist;
	float esp_name;
	float esp_box;
	float esp_object;
	float esp_team;
#endif

#if COMPILE_AIMBOT
	float aim_bot;
	float aim_auto_aim;
	float aim_auto_shoot;
	float aim_key;
	float aim_fov;
	float aim_mode;
	float aim_shoot;
	float aim_spot;
	float aim_team;
	float aim_smooth;
	float aim_triggerbot;
	float aim_draw;
	float aim_lock;
	float aim_prediction;
	float aim_method;
#endif

#if COMPILE_MISC
	float misc_bunnyhop;
	float misc_spy_cloak;
	float misc_spy_disg;
	float misc_taunt;
	float misc_auto_bs;
#endif

	float misc_menu_x;
	float misc_menu_y;
	float misc_menu_w;
};
//===================================================================================
extern CControlVariables gCvars;
//===================================================================================
#endif